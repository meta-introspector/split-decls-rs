// Generated macro for impl_43 (impl)
macro_rules! Depcrate_rawimpl_43 {
() => {
// Module: crate::raw
// Provides: {"impl_43"}
// Dependencies: {}
impl < F , T , S , M > RawTask < F , T , S , M > { pub (crate) const TASK_LAYOUT : TaskLayout = Self :: eval_task_layout () ; # [doc = " Computes the memory layout for a task."] # [inline] const fn eval_task_layout () -> TaskLayout { let layout_header = Layout :: new :: < HeaderWithMetadata < M > > () ; let layout_s = Layout :: new :: < S > () ; let layout_f = Layout :: new :: < F > () ; let layout_r = Layout :: new :: < Result < T , Panic > > () ; let size_union = max (layout_f . size () , layout_r . size ()) ; let align_union = max (layout_f . align () , layout_r . align ()) ; let layout_union = Layout :: from_size_align (size_union , align_union) ; let layout = layout_header ; let (layout , offset_s) = leap_unwrap ! (layout . extend (layout_s)) ; let (layout , offset_union) = leap_unwrap ! (layout . extend (layout_union)) ; let offset_f = offset_union ; let offset_r = offset_union ; TaskLayout { layout : unsafe { layout . into_std () } , offset_s , offset_f , offset_r , } } }
};
}
