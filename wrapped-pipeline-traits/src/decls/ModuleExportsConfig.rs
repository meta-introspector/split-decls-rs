macro_rules! ModuleExportsConfig {
    () => {
        # [derive (Debug , Deserialize , Clone)] pub struct ModuleExportsConfig { pub modules : Option < Vec < String > > , }
    };
}

ModuleExportsConfig!()