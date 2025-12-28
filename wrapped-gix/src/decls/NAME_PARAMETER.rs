macro_rules! deps {
    () => {
        SubSectionRequirement!();
    };
}

macro_rules! NAME_PARAMETER {
    () => {
        deps!();
        const NAME_PARAMETER : Option < SubSectionRequirement > = Some (SubSectionRequirement :: Parameter ("name")) ;
    };
}

NAME_PARAMETER!()