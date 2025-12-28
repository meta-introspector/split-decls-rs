macro_rules! deps {
    () => {
        Connection!();
        VTabLog!();
        Result!();
    };
}

macro_rules! load_module {
    () => {
        deps!();
        # [doc = " Register the \"vtablog\" module."] pub fn load_module (conn : & Connection) -> Result < () > { let aux : Option < () > = None ; conn . create_module (c"vtablog" , update_module :: < VTabLog > () , aux) }
    };
}

load_module!();