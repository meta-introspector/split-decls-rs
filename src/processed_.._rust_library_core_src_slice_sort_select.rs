// SRC: ../rust/library/core/src/slice/sort/select.rs
/* AST_META: AST_ID=1 | TYPE=STATEMENT | COMPLEXITY=2 */
//! This module contains the implementation for `slice::select_nth_unstable`.
/* AST_META: AST_ID=2 | TYPE=STATEMENT | COMPLEXITY=2 */
//! It uses an introselect algorithm based on ipnsort by Lukas Bergdoll and Orson Peters,
/* AST_META: AST_ID=3 | TYPE=STATEMENT | COMPLEXITY=2 */
//! published at: <https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort>
/* AST_META: AST_ID=4 | TYPE=STATEMENT | COMPLEXITY=1 */
//!
/* AST_META: AST_ID=5 | TYPE=STATEMENT | COMPLEXITY=2 */
//! The fallback algorithm used for introselect is Median of Medians using Tukey's Ninther
/* AST_META: AST_ID=6 | TYPE=STATEMENT | COMPLEXITY=2 */
//! for pivot selection. Using this as a fallback ensures O(n) worst case running time with
/* AST_META: AST_ID=7 | TYPE=STATEMENT | COMPLEXITY=2 */
//! better performance than one would get using heapsort as fallback.

/* AST_META: AST_ID=8 | TYPE=USE | COMPLEXITY=1 */
use crate::cfg_select;
/* AST_META: AST_ID=9 | TYPE=USE | COMPLEXITY=2 */
use crate::mem::{self, SizedTypeProperties};
/* AST_META: AST_ID=10 | TYPE=STATEMENT | COMPLEXITY=1 */
#[cfg(not(feature = "optimize_for_size"))]
/* AST_META: AST_ID=11 | TYPE=USE | COMPLEXITY=2 */
use crate::slice::sort::shared::pivot::choose_pivot;
/* AST_META: AST_ID=12 | TYPE=USE | COMPLEXITY=2 */
use crate::slice::sort::shared::smallsort::insertion_sort_shift_left;
/* AST_META: AST_ID=13 | TYPE=USE | COMPLEXITY=2 */
use crate::slice::sort::unstable::quicksort::partition;

/* AST_META: AST_ID=14 | TYPE=STATEMENT | COMPLEXITY=2 */
/// Reorders the slice such that the element at `index` is at its final sorted position.
/* AST_META: AST_ID=15 | TYPE=STATEMENT | COMPLEXITY=1 */
pub(crate) fn partition_at_index<T, F>(
/* AST_META: AST_ID=16 | TYPE=STATEMENT | COMPLEXITY=1 */
    v: &mut [T],
/* AST_META: AST_ID=17 | TYPE=STATEMENT | COMPLEXITY=1 */
    index: usize,
/* AST_META: AST_ID=18 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut is_less: F,
/* AST_META: AST_ID=19 | TYPE=STATEMENT | COMPLEXITY=1 */
) -> (&mut [T], &mut T, &mut [T])
/* AST_META: AST_ID=20 | TYPE=STATEMENT | COMPLEXITY=1 */
where
/* AST_META: AST_ID=21 | TYPE=STATEMENT | COMPLEXITY=1 */
    F: FnMut(&T, &T) -> bool,
/* AST_META: AST_ID=22 | TYPE=STATEMENT | COMPLEXITY=2 */
{
/* AST_META: AST_ID=23 | TYPE=STATEMENT | COMPLEXITY=1 */
    let len = v.len();

/* AST_META: AST_ID=24 | TYPE=STATEMENT | COMPLEXITY=1 */
    // Puts a lower limit of 1 on `len`.
/* AST_META: AST_ID=25 | TYPE=STATEMENT | COMPLEXITY=4 */
    if index >= len {
/* AST_META: AST_ID=26 | TYPE=STATEMENT | COMPLEXITY=4 */
        panic!("partition_at_index index {} greater than length of slice {}", index, len);
/* AST_META: AST_ID=27 | TYPE=STATEMENT | COMPLEXITY=1 */
    }

/* AST_META: AST_ID=28 | TYPE=STATEMENT | COMPLEXITY=4 */
    if T::IS_ZST {
/* AST_META: AST_ID=29 | TYPE=STATEMENT | COMPLEXITY=2 */
        // Sorting has no meaningful behavior on zero-sized types. Do nothing.
/* AST_META: AST_ID=30 | TYPE=STATEMENT | COMPLEXITY=4 */
    } else if index == len - 1 {
/* AST_META: AST_ID=31 | TYPE=STATEMENT | COMPLEXITY=2 */
        // Find max element and place it in the last position of the array. We're free to use
/* AST_META: AST_ID=32 | TYPE=STATEMENT | COMPLEXITY=2 */
        // `unwrap()` here because we checked that `v` is not empty.
/* AST_META: AST_ID=33 | TYPE=STATEMENT | COMPLEXITY=2 */
        let max_idx = max_index(v, &mut is_less).unwrap();
/* AST_META: AST_ID=34 | TYPE=STATEMENT | COMPLEXITY=1 */
        v.swap(max_idx, index);
/* AST_META: AST_ID=35 | TYPE=STATEMENT | COMPLEXITY=4 */
    } else if index == 0 {
/* AST_META: AST_ID=36 | TYPE=STATEMENT | COMPLEXITY=2 */
        // Find min element and place it in the first position of the array. We're free to use
/* AST_META: AST_ID=37 | TYPE=STATEMENT | COMPLEXITY=2 */
        // `unwrap()` here because we checked that `v` is not empty.
/* AST_META: AST_ID=38 | TYPE=STATEMENT | COMPLEXITY=2 */
        let min_idx = min_index(v, &mut is_less).unwrap();
/* AST_META: AST_ID=39 | TYPE=STATEMENT | COMPLEXITY=1 */
        v.swap(min_idx, index);
/* AST_META: AST_ID=40 | TYPE=STATEMENT | COMPLEXITY=2 */
    } else {
/* AST_META: AST_ID=41 | TYPE=STATEMENT | COMPLEXITY=2 */
        cfg_select! {
/* AST_META: AST_ID=42 | TYPE=STATEMENT | COMPLEXITY=2 */
            feature = "optimize_for_size" => {
/* AST_META: AST_ID=43 | TYPE=STATEMENT | COMPLEXITY=2 */
                median_of_medians(v, &mut is_less, index);
/* AST_META: AST_ID=44 | TYPE=STATEMENT | COMPLEXITY=1 */
            }
/* AST_META: AST_ID=45 | TYPE=STATEMENT | COMPLEXITY=2 */
            _ => {
/* AST_META: AST_ID=46 | TYPE=STATEMENT | COMPLEXITY=2 */
                partition_at_index_loop(v, index, None, &mut is_less);
/* AST_META: AST_ID=47 | TYPE=STATEMENT | COMPLEXITY=1 */
            }
/* AST_META: AST_ID=48 | TYPE=STATEMENT | COMPLEXITY=1 */
        }
/* AST_META: AST_ID=49 | TYPE=STATEMENT | COMPLEXITY=1 */
    }

/* AST_META: AST_ID=50 | TYPE=STATEMENT | COMPLEXITY=1 */
    let (left, right) = v.split_at_mut(index);
/* AST_META: AST_ID=51 | TYPE=STATEMENT | COMPLEXITY=1 */
    let (pivot, right) = right.split_at_mut(1);
/* AST_META: AST_ID=52 | TYPE=STATEMENT | COMPLEXITY=1 */
    let pivot = &mut pivot[0];
/* AST_META: AST_ID=53 | TYPE=STATEMENT | COMPLEXITY=1 */
    (left, pivot, right)
/* AST_META: AST_ID=54 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=55 | TYPE=STATEMENT | COMPLEXITY=2 */
// For small sub-slices it's faster to use a dedicated small-sort, but because it is only called at
/* AST_META: AST_ID=56 | TYPE=STATEMENT | COMPLEXITY=2 */
// most once, it doesn't make sense to use something more sophisticated than insertion-sort.
/* AST_META: AST_ID=57 | TYPE=STATEMENT | COMPLEXITY=1 */
const INSERTION_SORT_THRESHOLD: usize = 16;

/* AST_META: AST_ID=58 | TYPE=STATEMENT | COMPLEXITY=1 */
#[cfg(not(feature = "optimize_for_size"))]
/* AST_META: AST_ID=59 | TYPE=FUNCTION | COMPLEXITY=1 */
fn partition_at_index_loop<'a, T, F>(
/* AST_META: AST_ID=60 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut v: &'a mut [T],
/* AST_META: AST_ID=61 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut index: usize,
/* AST_META: AST_ID=62 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut ancestor_pivot: Option<&'a T>,
/* AST_META: AST_ID=63 | TYPE=STATEMENT | COMPLEXITY=1 */
    is_less: &mut F,
/* AST_META: AST_ID=64 | TYPE=STATEMENT | COMPLEXITY=1 */
) where
/* AST_META: AST_ID=65 | TYPE=STATEMENT | COMPLEXITY=1 */
    F: FnMut(&T, &T) -> bool,
/* AST_META: AST_ID=66 | TYPE=STATEMENT | COMPLEXITY=2 */
{
/* AST_META: AST_ID=67 | TYPE=STATEMENT | COMPLEXITY=2 */
    // Limit the amount of iterations and fall back to fast deterministic selection to ensure O(n)
/* AST_META: AST_ID=68 | TYPE=STATEMENT | COMPLEXITY=2 */
    // worst case running time. This limit needs to be constant, because using `ilog2(len)` like in
/* AST_META: AST_ID=69 | TYPE=STATEMENT | COMPLEXITY=2 */
    // `sort` would result in O(n log n) time complexity. The exact value of the limit is chosen
/* AST_META: AST_ID=70 | TYPE=STATEMENT | COMPLEXITY=2 */
    // somewhat arbitrarily, but for most inputs bad pivot selections should be relatively rare, so
/* AST_META: AST_ID=71 | TYPE=STATEMENT | COMPLEXITY=2 */
    // the limit is reached for sub-slices len / (2^limit or less). Which makes the remaining work
/* AST_META: AST_ID=72 | TYPE=STATEMENT | COMPLEXITY=2 */
    // with the fallback minimal in relative terms.
/* AST_META: AST_ID=73 | TYPE=STATEMENT | COMPLEXITY=1 */
    let mut limit = 16;

/* AST_META: AST_ID=74 | TYPE=STATEMENT | COMPLEXITY=2 */
    loop {
/* AST_META: AST_ID=75 | TYPE=STATEMENT | COMPLEXITY=4 */
        if v.len() <= INSERTION_SORT_THRESHOLD {
/* AST_META: AST_ID=76 | TYPE=STATEMENT | COMPLEXITY=4 */
            if v.len() >= 2 {
/* AST_META: AST_ID=77 | TYPE=STATEMENT | COMPLEXITY=2 */
                insertion_sort_shift_left(v, 1, is_less);
/* AST_META: AST_ID=78 | TYPE=STATEMENT | COMPLEXITY=1 */
            }
/* AST_META: AST_ID=79 | TYPE=STATEMENT | COMPLEXITY=1 */
            return;
/* AST_META: AST_ID=80 | TYPE=STATEMENT | COMPLEXITY=1 */
        }

/* AST_META: AST_ID=81 | TYPE=STATEMENT | COMPLEXITY=4 */
        if limit == 0 {
/* AST_META: AST_ID=82 | TYPE=STATEMENT | COMPLEXITY=1 */
            median_of_medians(v, is_less, index);
/* AST_META: AST_ID=83 | TYPE=STATEMENT | COMPLEXITY=1 */
            return;
/* AST_META: AST_ID=84 | TYPE=STATEMENT | COMPLEXITY=1 */
        }

/* AST_META: AST_ID=85 | TYPE=STATEMENT | COMPLEXITY=1 */
        limit -= 1;

/* AST_META: AST_ID=86 | TYPE=STATEMENT | COMPLEXITY=1 */
        // Choose a pivot
/* AST_META: AST_ID=87 | TYPE=STATEMENT | COMPLEXITY=1 */
        let pivot_pos = choose_pivot(v, is_less);

/* AST_META: AST_ID=88 | TYPE=STATEMENT | COMPLEXITY=2 */
        // If the chosen pivot is equal to the predecessor, then it's the smallest element in the
/* AST_META: AST_ID=89 | TYPE=STATEMENT | COMPLEXITY=2 */
        // slice. Partition the slice into elements equal to and elements greater than the pivot.
/* AST_META: AST_ID=90 | TYPE=STATEMENT | COMPLEXITY=2 */
        // This case is usually hit when the slice contains many duplicate elements.
/* AST_META: AST_ID=91 | TYPE=STATEMENT | COMPLEXITY=4 */
        if let Some(p) = ancestor_pivot {
/* AST_META: AST_ID=92 | TYPE=STATEMENT | COMPLEXITY=1 */
            let pivot = &v[pivot_pos];

/* AST_META: AST_ID=93 | TYPE=STATEMENT | COMPLEXITY=4 */
            if !is_less(p, pivot) {
/* AST_META: AST_ID=94 | TYPE=STATEMENT | COMPLEXITY=2 */
                let num_lt = partition(v, pivot_pos, &mut |a, b| !is_less(b, a));

/* AST_META: AST_ID=95 | TYPE=STATEMENT | COMPLEXITY=2 */
                // Continue sorting elements greater than the pivot. We know that `mid` contains
/* AST_META: AST_ID=96 | TYPE=STATEMENT | COMPLEXITY=2 */
                // the pivot. So we can continue after `mid`.
/* AST_META: AST_ID=97 | TYPE=STATEMENT | COMPLEXITY=1 */
                let mid = num_lt + 1;

/* AST_META: AST_ID=98 | TYPE=STATEMENT | COMPLEXITY=2 */
                // If we've passed our index, then we're good.
/* AST_META: AST_ID=99 | TYPE=STATEMENT | COMPLEXITY=4 */
                if mid > index {
/* AST_META: AST_ID=100 | TYPE=STATEMENT | COMPLEXITY=1 */
                    return;
/* AST_META: AST_ID=101 | TYPE=STATEMENT | COMPLEXITY=1 */
                }

/* AST_META: AST_ID=102 | TYPE=STATEMENT | COMPLEXITY=1 */
                v = &mut v[mid..];
/* AST_META: AST_ID=103 | TYPE=STATEMENT | COMPLEXITY=1 */
                index = index - mid;
/* AST_META: AST_ID=104 | TYPE=STATEMENT | COMPLEXITY=1 */
                ancestor_pivot = None;
/* AST_META: AST_ID=105 | TYPE=STATEMENT | COMPLEXITY=1 */
                continue;
/* AST_META: AST_ID=106 | TYPE=STATEMENT | COMPLEXITY=1 */
            }
/* AST_META: AST_ID=107 | TYPE=STATEMENT | COMPLEXITY=1 */
        }

/* AST_META: AST_ID=108 | TYPE=STATEMENT | COMPLEXITY=2 */
        let mid = partition(v, pivot_pos, is_less);

/* AST_META: AST_ID=109 | TYPE=STATEMENT | COMPLEXITY=2 */
        // Split the slice into `left`, `pivot`, and `right`.
/* AST_META: AST_ID=110 | TYPE=STATEMENT | COMPLEXITY=1 */
        let (left, right) = v.split_at_mut(mid);
/* AST_META: AST_ID=111 | TYPE=STATEMENT | COMPLEXITY=2 */
        let (pivot, right) = right.split_at_mut(1);
/* AST_META: AST_ID=112 | TYPE=STATEMENT | COMPLEXITY=1 */
        let pivot = &pivot[0];

/* AST_META: AST_ID=113 | TYPE=STATEMENT | COMPLEXITY=4 */
        if mid < index {
/* AST_META: AST_ID=114 | TYPE=STATEMENT | COMPLEXITY=1 */
            v = right;
/* AST_META: AST_ID=115 | TYPE=STATEMENT | COMPLEXITY=1 */
            index = index - mid - 1;
/* AST_META: AST_ID=116 | TYPE=STATEMENT | COMPLEXITY=1 */
            ancestor_pivot = Some(pivot);
/* AST_META: AST_ID=117 | TYPE=STATEMENT | COMPLEXITY=4 */
        } else if mid > index {
/* AST_META: AST_ID=118 | TYPE=STATEMENT | COMPLEXITY=1 */
            v = left;
/* AST_META: AST_ID=119 | TYPE=STATEMENT | COMPLEXITY=2 */
        } else {
/* AST_META: AST_ID=120 | TYPE=STATEMENT | COMPLEXITY=2 */
            // If mid == index, then we're done, since partition() guaranteed that all elements
/* AST_META: AST_ID=121 | TYPE=STATEMENT | COMPLEXITY=2 */
            // after mid are greater than or equal to mid.
/* AST_META: AST_ID=122 | TYPE=STATEMENT | COMPLEXITY=1 */
            return;
/* AST_META: AST_ID=123 | TYPE=STATEMENT | COMPLEXITY=1 */
        }
/* AST_META: AST_ID=124 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=125 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=126 | TYPE=STATEMENT | COMPLEXITY=2 */
/// Helper function that returns the index of the minimum element in the slice using the given
/* AST_META: AST_ID=127 | TYPE=STATEMENT | COMPLEXITY=1 */
/// comparator function
/* AST_META: AST_ID=128 | TYPE=FUNCTION | COMPLEXITY=3 */
fn min_index<T, F: FnMut(&T, &T) -> bool>(slice: &[T], is_less: &mut F) -> Option<usize> {
/* AST_META: AST_ID=129 | TYPE=STATEMENT | COMPLEXITY=1 */
    slice
/* AST_META: AST_ID=130 | TYPE=STATEMENT | COMPLEXITY=1 */
        .iter()
/* AST_META: AST_ID=131 | TYPE=STATEMENT | COMPLEXITY=1 */
        .enumerate()
/* AST_META: AST_ID=132 | TYPE=STATEMENT | COMPLEXITY=6 */
        .reduce(|acc, t| if is_less(t.1, acc.1) { t } else { acc })
/* AST_META: AST_ID=133 | TYPE=STATEMENT | COMPLEXITY=1 */
        .map(|(i, _)| i)
/* AST_META: AST_ID=134 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=135 | TYPE=STATEMENT | COMPLEXITY=2 */
/// Helper function that returns the index of the maximum element in the slice using the given
/* AST_META: AST_ID=136 | TYPE=STATEMENT | COMPLEXITY=1 */
/// comparator function
/* AST_META: AST_ID=137 | TYPE=FUNCTION | COMPLEXITY=3 */
fn max_index<T, F: FnMut(&T, &T) -> bool>(slice: &[T], is_less: &mut F) -> Option<usize> {
/* AST_META: AST_ID=138 | TYPE=STATEMENT | COMPLEXITY=1 */
    slice
/* AST_META: AST_ID=139 | TYPE=STATEMENT | COMPLEXITY=1 */
        .iter()
/* AST_META: AST_ID=140 | TYPE=STATEMENT | COMPLEXITY=1 */
        .enumerate()
/* AST_META: AST_ID=141 | TYPE=STATEMENT | COMPLEXITY=6 */
        .reduce(|acc, t| if is_less(acc.1, t.1) { t } else { acc })
/* AST_META: AST_ID=142 | TYPE=STATEMENT | COMPLEXITY=1 */
        .map(|(i, _)| i)
/* AST_META: AST_ID=143 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=144 | TYPE=STATEMENT | COMPLEXITY=2 */
/// Selection algorithm to select the k-th element from the slice in guaranteed O(n) time.
/* AST_META: AST_ID=145 | TYPE=STATEMENT | COMPLEXITY=2 */
/// This is essentially a quickselect that uses Tukey's Ninther for pivot selection
/* AST_META: AST_ID=146 | TYPE=FUNCTION | COMPLEXITY=3 */
fn median_of_medians<T, F: FnMut(&T, &T) -> bool>(mut v: &mut [T], is_less: &mut F, mut k: usize) {
/* AST_META: AST_ID=147 | TYPE=STATEMENT | COMPLEXITY=2 */
    // Since this function isn't public, it should never be called with an out-of-bounds index.
/* AST_META: AST_ID=148 | TYPE=STATEMENT | COMPLEXITY=1 */
    debug_assert!(k < v.len());

/* AST_META: AST_ID=149 | TYPE=STATEMENT | COMPLEXITY=2 */
    // If T is as ZST, `partition_at_index` will already return early.
/* AST_META: AST_ID=150 | TYPE=STATEMENT | COMPLEXITY=1 */
    debug_assert!(!T::IS_ZST);

/* AST_META: AST_ID=151 | TYPE=STATEMENT | COMPLEXITY=2 */
    // We now know that `k < v.len() <= isize::MAX`
/* AST_META: AST_ID=152 | TYPE=STATEMENT | COMPLEXITY=2 */
    loop {
/* AST_META: AST_ID=153 | TYPE=STATEMENT | COMPLEXITY=4 */
        if v.len() <= INSERTION_SORT_THRESHOLD {
/* AST_META: AST_ID=154 | TYPE=STATEMENT | COMPLEXITY=4 */
            if v.len() >= 2 {
/* AST_META: AST_ID=155 | TYPE=STATEMENT | COMPLEXITY=2 */
                insertion_sort_shift_left(v, 1, is_less);
/* AST_META: AST_ID=156 | TYPE=STATEMENT | COMPLEXITY=1 */
            }

/* AST_META: AST_ID=157 | TYPE=STATEMENT | COMPLEXITY=1 */
            return;
/* AST_META: AST_ID=158 | TYPE=STATEMENT | COMPLEXITY=1 */
        }

/* AST_META: AST_ID=159 | TYPE=STATEMENT | COMPLEXITY=3 */
        // `median_of_{minima,maxima}` can't handle the extreme cases of the first/last element,
/* AST_META: AST_ID=160 | TYPE=STATEMENT | COMPLEXITY=2 */
        // so we catch them here and just do a linear search.
/* AST_META: AST_ID=161 | TYPE=STATEMENT | COMPLEXITY=4 */
        if k == v.len() - 1 {
/* AST_META: AST_ID=162 | TYPE=STATEMENT | COMPLEXITY=2 */
            // Find max element and place it in the last position of the array. We're free to use
/* AST_META: AST_ID=163 | TYPE=STATEMENT | COMPLEXITY=2 */
            // `unwrap()` here because we know v must not be empty.
/* AST_META: AST_ID=164 | TYPE=STATEMENT | COMPLEXITY=2 */
            let max_idx = max_index(v, is_less).unwrap();
/* AST_META: AST_ID=165 | TYPE=STATEMENT | COMPLEXITY=1 */
            v.swap(max_idx, k);
/* AST_META: AST_ID=166 | TYPE=STATEMENT | COMPLEXITY=1 */
            return;
/* AST_META: AST_ID=167 | TYPE=STATEMENT | COMPLEXITY=4 */
        } else if k == 0 {
/* AST_META: AST_ID=168 | TYPE=STATEMENT | COMPLEXITY=2 */
            // Find min element and place it in the first position of the array. We're free to use
/* AST_META: AST_ID=169 | TYPE=STATEMENT | COMPLEXITY=2 */
            // `unwrap()` here because we know v must not be empty.
/* AST_META: AST_ID=170 | TYPE=STATEMENT | COMPLEXITY=2 */
            let min_idx = min_index(v, is_less).unwrap();
/* AST_META: AST_ID=171 | TYPE=STATEMENT | COMPLEXITY=1 */
            v.swap(min_idx, k);
/* AST_META: AST_ID=172 | TYPE=STATEMENT | COMPLEXITY=1 */
            return;
/* AST_META: AST_ID=173 | TYPE=STATEMENT | COMPLEXITY=1 */
        }

/* AST_META: AST_ID=174 | TYPE=STATEMENT | COMPLEXITY=1 */
        let p = median_of_ninthers(v, is_less);

/* AST_META: AST_ID=175 | TYPE=STATEMENT | COMPLEXITY=4 */
        if p == k {
/* AST_META: AST_ID=176 | TYPE=STATEMENT | COMPLEXITY=1 */
            return;
/* AST_META: AST_ID=177 | TYPE=STATEMENT | COMPLEXITY=4 */
        } else if p > k {
/* AST_META: AST_ID=178 | TYPE=STATEMENT | COMPLEXITY=1 */
            v = &mut v[..p];
/* AST_META: AST_ID=179 | TYPE=STATEMENT | COMPLEXITY=2 */
        } else {
/* AST_META: AST_ID=180 | TYPE=STATEMENT | COMPLEXITY=2 */
            // Since `p < k < v.len()`, `p + 1` doesn't overflow and is
/* AST_META: AST_ID=181 | TYPE=STATEMENT | COMPLEXITY=1 */
            // a valid index into the slice.
/* AST_META: AST_ID=182 | TYPE=STATEMENT | COMPLEXITY=1 */
            v = &mut v[p + 1..];
/* AST_META: AST_ID=183 | TYPE=STATEMENT | COMPLEXITY=1 */
            k -= p + 1;
/* AST_META: AST_ID=184 | TYPE=STATEMENT | COMPLEXITY=1 */
        }
/* AST_META: AST_ID=185 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=186 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=187 | TYPE=STATEMENT | COMPLEXITY=2 */
// Optimized for when `k` lies somewhere in the middle of the slice. Selects a pivot
/* AST_META: AST_ID=188 | TYPE=STATEMENT | COMPLEXITY=2 */
// as close as possible to the median of the slice. For more details on how the algorithm
/* AST_META: AST_ID=189 | TYPE=STATEMENT | COMPLEXITY=3 */
// operates, refer to the paper <https://drops.dagstuhl.de/opus/volltexte/2017/7612/pdf/LIPIcs-SEA-2017-24.pdf>.
/* AST_META: AST_ID=190 | TYPE=FUNCTION | COMPLEXITY=3 */
fn median_of_ninthers<T, F: FnMut(&T, &T) -> bool>(v: &mut [T], is_less: &mut F) -> usize {
/* AST_META: AST_ID=191 | TYPE=STATEMENT | COMPLEXITY=2 */
    // use `saturating_mul` so the multiplication doesn't overflow on 16-bit platforms.
/* AST_META: AST_ID=192 | TYPE=STATEMENT | COMPLEXITY=4 */
    let frac = if v.len() <= 1024 {
/* AST_META: AST_ID=193 | TYPE=STATEMENT | COMPLEXITY=1 */
        v.len() / 12
/* AST_META: AST_ID=194 | TYPE=STATEMENT | COMPLEXITY=5 */
    } else if v.len() <= 128_usize.saturating_mul(1024) {
/* AST_META: AST_ID=195 | TYPE=STATEMENT | COMPLEXITY=1 */
        v.len() / 64
/* AST_META: AST_ID=196 | TYPE=STATEMENT | COMPLEXITY=2 */
    } else {
/* AST_META: AST_ID=197 | TYPE=STATEMENT | COMPLEXITY=1 */
        v.len() / 1024
/* AST_META: AST_ID=198 | TYPE=STATEMENT | COMPLEXITY=1 */
    };

/* AST_META: AST_ID=199 | TYPE=STATEMENT | COMPLEXITY=1 */
    let pivot = frac / 2;
/* AST_META: AST_ID=200 | TYPE=STATEMENT | COMPLEXITY=1 */
    let lo = v.len() / 2 - pivot;
/* AST_META: AST_ID=201 | TYPE=STATEMENT | COMPLEXITY=1 */
    let hi = frac + lo;
/* AST_META: AST_ID=202 | TYPE=STATEMENT | COMPLEXITY=1 */
    let gap = (v.len() - 9 * frac) / 4;
/* AST_META: AST_ID=203 | TYPE=STATEMENT | COMPLEXITY=1 */
    let mut a = lo - 4 * frac - gap;
/* AST_META: AST_ID=204 | TYPE=STATEMENT | COMPLEXITY=1 */
    let mut b = hi + gap;
/* AST_META: AST_ID=205 | TYPE=STATEMENT | COMPLEXITY=2 */
    for i in lo..hi {
/* AST_META: AST_ID=206 | TYPE=STATEMENT | COMPLEXITY=2 */
        ninther(v, is_less, a, i - frac, b, a + 1, i, b + 1, a + 2, i + frac, b + 2);
/* AST_META: AST_ID=207 | TYPE=STATEMENT | COMPLEXITY=1 */
        a += 3;
/* AST_META: AST_ID=208 | TYPE=STATEMENT | COMPLEXITY=1 */
        b += 3;
/* AST_META: AST_ID=209 | TYPE=STATEMENT | COMPLEXITY=1 */
    }

/* AST_META: AST_ID=210 | TYPE=STATEMENT | COMPLEXITY=2 */
    median_of_medians(&mut v[lo..lo + frac], is_less, pivot);

/* AST_META: AST_ID=211 | TYPE=STATEMENT | COMPLEXITY=1 */
    partition(v, lo + pivot, is_less)
/* AST_META: AST_ID=212 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=213 | TYPE=STATEMENT | COMPLEXITY=2 */
/// Moves around the 9 elements at the indices a..i, such that
/* AST_META: AST_ID=214 | TYPE=STATEMENT | COMPLEXITY=2 */
/// `v[d]` contains the median of the 9 elements and the other
/* AST_META: AST_ID=215 | TYPE=STATEMENT | COMPLEXITY=1 */
/// elements are partitioned around it.
/* AST_META: AST_ID=216 | TYPE=FUNCTION | COMPLEXITY=1 */
fn ninther<T, F: FnMut(&T, &T) -> bool>(
/* AST_META: AST_ID=217 | TYPE=STATEMENT | COMPLEXITY=1 */
    v: &mut [T],
/* AST_META: AST_ID=218 | TYPE=STATEMENT | COMPLEXITY=1 */
    is_less: &mut F,
/* AST_META: AST_ID=219 | TYPE=STATEMENT | COMPLEXITY=1 */
    a: usize,
/* AST_META: AST_ID=220 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut b: usize,
/* AST_META: AST_ID=221 | TYPE=STATEMENT | COMPLEXITY=1 */
    c: usize,
/* AST_META: AST_ID=222 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut d: usize,
/* AST_META: AST_ID=223 | TYPE=STATEMENT | COMPLEXITY=1 */
    e: usize,
/* AST_META: AST_ID=224 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut f: usize,
/* AST_META: AST_ID=225 | TYPE=STATEMENT | COMPLEXITY=1 */
    g: usize,
/* AST_META: AST_ID=226 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut h: usize,
/* AST_META: AST_ID=227 | TYPE=STATEMENT | COMPLEXITY=1 */
    i: usize,
/* AST_META: AST_ID=228 | TYPE=STATEMENT | COMPLEXITY=2 */
) {
/* AST_META: AST_ID=229 | TYPE=STATEMENT | COMPLEXITY=1 */
    b = median_idx(v, is_less, a, b, c);
/* AST_META: AST_ID=230 | TYPE=STATEMENT | COMPLEXITY=1 */
    h = median_idx(v, is_less, g, h, i);
/* AST_META: AST_ID=231 | TYPE=STATEMENT | COMPLEXITY=4 */
    if is_less(&v[h], &v[b]) {
/* AST_META: AST_ID=232 | TYPE=STATEMENT | COMPLEXITY=1 */
        mem::swap(&mut b, &mut h);
/* AST_META: AST_ID=233 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=234 | TYPE=STATEMENT | COMPLEXITY=4 */
    if is_less(&v[f], &v[d]) {
/* AST_META: AST_ID=235 | TYPE=STATEMENT | COMPLEXITY=1 */
        mem::swap(&mut d, &mut f);
/* AST_META: AST_ID=236 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=237 | TYPE=STATEMENT | COMPLEXITY=4 */
    if is_less(&v[e], &v[d]) {
/* AST_META: AST_ID=238 | TYPE=STATEMENT | COMPLEXITY=1 */
        // do nothing
/* AST_META: AST_ID=239 | TYPE=STATEMENT | COMPLEXITY=4 */
    } else if is_less(&v[f], &v[e]) {
/* AST_META: AST_ID=240 | TYPE=STATEMENT | COMPLEXITY=1 */
        d = f;
/* AST_META: AST_ID=241 | TYPE=STATEMENT | COMPLEXITY=2 */
    } else {
/* AST_META: AST_ID=242 | TYPE=STATEMENT | COMPLEXITY=4 */
        if is_less(&v[e], &v[b]) {
/* AST_META: AST_ID=243 | TYPE=STATEMENT | COMPLEXITY=1 */
            v.swap(e, b);
/* AST_META: AST_ID=244 | TYPE=STATEMENT | COMPLEXITY=4 */
        } else if is_less(&v[h], &v[e]) {
/* AST_META: AST_ID=245 | TYPE=STATEMENT | COMPLEXITY=1 */
            v.swap(e, h);
/* AST_META: AST_ID=246 | TYPE=STATEMENT | COMPLEXITY=1 */
        }
/* AST_META: AST_ID=247 | TYPE=STATEMENT | COMPLEXITY=1 */
        return;
/* AST_META: AST_ID=248 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=249 | TYPE=STATEMENT | COMPLEXITY=4 */
    if is_less(&v[d], &v[b]) {
/* AST_META: AST_ID=250 | TYPE=STATEMENT | COMPLEXITY=1 */
        d = b;
/* AST_META: AST_ID=251 | TYPE=STATEMENT | COMPLEXITY=4 */
    } else if is_less(&v[h], &v[d]) {
/* AST_META: AST_ID=252 | TYPE=STATEMENT | COMPLEXITY=1 */
        d = h;
/* AST_META: AST_ID=253 | TYPE=STATEMENT | COMPLEXITY=1 */
    }

/* AST_META: AST_ID=254 | TYPE=STATEMENT | COMPLEXITY=1 */
    v.swap(d, e);
/* AST_META: AST_ID=255 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=256 | TYPE=STATEMENT | COMPLEXITY=2 */
/// returns the index pointing to the median of the 3
/* AST_META: AST_ID=257 | TYPE=STATEMENT | COMPLEXITY=1 */
/// elements `v[a]`, `v[b]` and `v[c]`
/* AST_META: AST_ID=258 | TYPE=FUNCTION | COMPLEXITY=1 */
fn median_idx<T, F: FnMut(&T, &T) -> bool>(
/* AST_META: AST_ID=259 | TYPE=STATEMENT | COMPLEXITY=1 */
    v: &[T],
/* AST_META: AST_ID=260 | TYPE=STATEMENT | COMPLEXITY=1 */
    is_less: &mut F,
/* AST_META: AST_ID=261 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut a: usize,
/* AST_META: AST_ID=262 | TYPE=STATEMENT | COMPLEXITY=1 */
    b: usize,
/* AST_META: AST_ID=263 | TYPE=STATEMENT | COMPLEXITY=1 */
    mut c: usize,
/* AST_META: AST_ID=264 | TYPE=STATEMENT | COMPLEXITY=2 */
) -> usize {
/* AST_META: AST_ID=265 | TYPE=STATEMENT | COMPLEXITY=4 */
    if is_less(&v[c], &v[a]) {
/* AST_META: AST_ID=266 | TYPE=STATEMENT | COMPLEXITY=1 */
        mem::swap(&mut a, &mut c);
/* AST_META: AST_ID=267 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=268 | TYPE=STATEMENT | COMPLEXITY=4 */
    if is_less(&v[c], &v[b]) {
/* AST_META: AST_ID=269 | TYPE=STATEMENT | COMPLEXITY=1 */
        return c;
/* AST_META: AST_ID=270 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=271 | TYPE=STATEMENT | COMPLEXITY=4 */
    if is_less(&v[b], &v[a]) {
/* AST_META: AST_ID=272 | TYPE=STATEMENT | COMPLEXITY=1 */
        return a;
/* AST_META: AST_ID=273 | TYPE=STATEMENT | COMPLEXITY=1 */
    }
/* AST_META: AST_ID=274 | TYPE=STATEMENT | COMPLEXITY=1 */
    b
/* AST_META: AST_ID=275 | TYPE=STATEMENT | COMPLEXITY=1 */
}
