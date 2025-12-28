macro_rules! deps {
    () => {
        Any!();
        SubSectionRequirement!();
        Init!();
        Section!();
        Validate!();
    };
}

macro_rules! impl_759 {
    () => {
        deps!();
        # [doc = " Init other validate implementations"] impl < T : Validate > Any < T > { # [doc = " Create a new instance from `name` and `section`"] pub const fn new_with_validate (name : & 'static str , section : & 'static dyn Section , validate : T) -> Self { Any { name , section , subsection_requirement : Some (SubSectionRequirement :: Never) , link : None , note : None , validate , } } }
    };
}

impl_759!()