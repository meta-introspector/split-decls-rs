macro_rules! deps {
    () => {
        AixLinker!();
        Command!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'a > AixLinker < 'a > { fn new (cmd : Command , sess : & 'a Session) -> AixLinker < 'a > { AixLinker { cmd , sess , hinted_static : None } } fn hint_static (& mut self) { if self . hinted_static != Some (true) { self . link_arg ("-bstatic") ; self . hinted_static = Some (true) ; } } fn hint_dynamic (& mut self) { if self . hinted_static != Some (false) { self . link_arg ("-bdynamic") ; self . hinted_static = Some (false) ; } } fn build_dylib (& mut self , _out_filename : & Path) { self . link_args (& ["-bM:SRE" , "-bnoentry"]) ; self . link_arg ("-bexpfull") ; } }
    };
}

impl_129!();