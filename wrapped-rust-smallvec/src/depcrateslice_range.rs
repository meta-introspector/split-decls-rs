// Generated macro for slice_range (function)
macro_rules! Depcrateslice_range {
() => {
// Module: crate
// Provides: {"slice_range"}
// Dependencies: {}
# [inline] # [doc = " A local copy of [`core::slice::range`]. The latter function is unstable"] # [doc = " and thus cannot be used yet."] fn slice_range < R > (range : R , bounds : core :: ops :: RangeTo < usize >) -> core :: ops :: Range < usize > where R : core :: ops :: RangeBounds < usize > , { let len = bounds . end ; let start = match range . start_bound () { core :: ops :: Bound :: Included (& start) => start , core :: ops :: Bound :: Excluded (start) => start . checked_add (1) . unwrap_or_else (| | panic ! ("attempted to index slice from after maximum usize")) , core :: ops :: Bound :: Unbounded => 0 , } ; let end = match range . end_bound () { core :: ops :: Bound :: Included (end) => end . checked_add (1) . unwrap_or_else (| | panic ! ("attempted to index slice up to maximum usize")) , core :: ops :: Bound :: Excluded (& end) => end , core :: ops :: Bound :: Unbounded => len , } ; if start > end { panic ! ("slice index starts at {start} but ends at {end}") ; } if end > len { panic ! ("range end index {end} out of range for slice of length {len}") ; } core :: ops :: Range { start , end } }
};
}
