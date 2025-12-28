macro_rules! CPU_SUBTYPE_ANY {
    () => {
        # [doc = " When selecting a slice, ANY will pick the slice with the best"] # [doc = " grading for the selected cpu_type_t, unlike the \"ALL\" subtypes,"] # [doc = " which are the slices that can run on any hardware for that cpu type."] pub const CPU_SUBTYPE_ANY : u32 = ! 0 ;
    };
}

CPU_SUBTYPE_ANY!();