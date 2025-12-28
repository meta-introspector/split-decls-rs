macro_rules! deps {
    () => {
        MatchedArg!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl PartialEq for MatchedArg { fn eq (& self , other : & MatchedArg) -> bool { let MatchedArg { source : self_source , indices : self_indices , type_id : self_type_id , vals : _ , raw_vals : self_raw_vals , ignore_case : self_ignore_case , } = self ; let MatchedArg { source : other_source , indices : other_indices , type_id : other_type_id , vals : _ , raw_vals : other_raw_vals , ignore_case : other_ignore_case , } = other ; self_source == other_source && self_indices == other_indices && self_type_id == other_type_id && self_raw_vals == other_raw_vals && self_ignore_case == other_ignore_case } }
    };
}

impl_501!();