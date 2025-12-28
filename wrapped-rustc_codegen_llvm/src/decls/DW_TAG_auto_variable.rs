macro_rules! DW_TAG_auto_variable {
    () => {
        # [allow (non_upper_case_globals)] const DW_TAG_auto_variable : c_uint = 0x100 ;
    };
}

DW_TAG_auto_variable!();