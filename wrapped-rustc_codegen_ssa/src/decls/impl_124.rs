macro_rules! deps {
    () => {
        WasmLd!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a > WasmLd < 'a > { fn push_linker_plugin_lto_args (& mut self) { let opt_level = match self . sess . opts . optimize { config :: OptLevel :: No => "O0" , config :: OptLevel :: Less => "O1" , config :: OptLevel :: More => "O2" , config :: OptLevel :: Aggressive => "O3" , config :: OptLevel :: Size | config :: OptLevel :: SizeMin => "O2" , } ; self . link_arg (& format ! ("--lto-{opt_level}")) ; } }
    };
}

impl_124!();