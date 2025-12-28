macro_rules! deps {
    () => {
        TableLayout!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl TableLayout { # [inline] const fn new < T > () -> Self { let layout = Layout :: new :: < T > () ; Self { size : layout . size () , ctrl_align : if layout . align () > Group :: WIDTH { layout . align () } else { Group :: WIDTH } , } } # [inline] fn calculate_layout_for (self , buckets : usize) -> Option < (Layout , usize) > { debug_assert ! (buckets . is_power_of_two ()) ; let TableLayout { size , ctrl_align } = self ; let ctrl_offset = size . checked_mul (buckets) ? . checked_add (ctrl_align - 1) ? & ! (ctrl_align - 1) ; let len = ctrl_offset . checked_add (buckets + Group :: WIDTH) ? ; if len > isize :: MAX as usize - (ctrl_align - 1) { return None ; } Some ((unsafe { Layout :: from_size_align_unchecked (len , ctrl_align) } , ctrl_offset ,)) } }
    };
}

impl_44!();