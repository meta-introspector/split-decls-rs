macro_rules! deps {
    () => {
        JoiningType!();
    };
}

macro_rules! macro_145 {
    () => {
        deps!();
        create_const_array ! { # [allow (missing_docs)] # [allow (non_upper_case_globals)] impl JoiningType { pub const NonJoining : JoiningType = JoiningType (0) ; pub const JoinCausing : JoiningType = JoiningType (1) ; pub const DualJoining : JoiningType = JoiningType (2) ; pub const LeftJoining : JoiningType = JoiningType (3) ; pub const RightJoining : JoiningType = JoiningType (4) ; pub const Transparent : JoiningType = JoiningType (5) ; } }
    };
}

macro_145!()