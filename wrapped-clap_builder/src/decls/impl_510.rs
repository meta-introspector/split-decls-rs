macro_rules! deps {
    () => {
        Parser!();
        Command!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < 'cmd > Parser < 'cmd > { pub (crate) fn new (cmd : & 'cmd mut Command) -> Self { Parser { cmd , cur_idx : Cell :: new (0) , flag_subcmd_at : None , flag_subcmd_skip : 0 , } } }
    };
}

impl_510!()