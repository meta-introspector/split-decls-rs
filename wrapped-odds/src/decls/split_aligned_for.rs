macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! split_aligned_for {
    () => {
        deps!();
        # [doc = " Split the input slice into three chunks,"] # [doc = " so that the middle chunk is a slice of a larger \"block size\""] # [doc = " (for example T could be u64) that is correctly aligned for `T`."] # [doc = ""] # [doc = " The first and last returned slices are the remaining head and tail"] # [doc = " parts that could not be baked into `&[T]`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate odds;"] # [doc = " use odds::slice::split_aligned_for;"] # [doc = ""] # [doc = " fn count_ones(data: &[u8]) -> u32 {"] # [doc = "     let mut total = 0;"] # [doc = "     let (head, mid, tail) = split_aligned_for::<[u64; 2]>(data);"] # [doc = "     total += head.iter().map(|x| x.count_ones()).sum::<u32>();"] # [doc = "     total += mid.iter().map(|x| x[0].count_ones() + x[1].count_ones()).sum::<u32>();"] # [doc = "     total += tail.iter().map(|x| x.count_ones()).sum::<u32>();"] # [doc = "     total"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     assert_eq!(count_ones(&vec![3u8; 127]), 127 * 2);"] # [doc = " }"] # [doc = " ```"] pub fn split_aligned_for < T : Pod > (data : & [u8]) -> (& [u8] , & [T] , & [u8]) { let ptr = data . as_ptr () ; let align_t = align_of :: < T > () ; let size_t = size_of :: < T > () ; let align_ptr = ptr as usize & (align_t - 1) ; let prefix = if align_ptr == 0 { 0 } else { align_t - align_ptr } ; let t_len ; if prefix > data . len () { t_len = 0 ; } else { t_len = (data . len () - prefix) / size_t ; } unsafe { (from_raw_parts (ptr , prefix) , from_raw_parts (ptr . offset (prefix as isize) as * const T , t_len) , from_raw_parts (ptr . offset ((prefix + t_len * size_t) as isize) , data . len () - t_len * size_t - prefix ,) ,) } }
    };
}

split_aligned_for!()