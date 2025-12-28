macro_rules! deps {
    () => {
        ArgMatches!();
    };
}

macro_rules! SubCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub (crate) struct SubCommand { pub (crate) name : String , pub (crate) matches : ArgMatches , }
    };
}

SubCommand!()