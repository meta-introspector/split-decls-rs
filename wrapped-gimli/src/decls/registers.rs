macro_rules! deps {
    () => {
        Register!();
    };
}

macro_rules! registers {
    () => {
        deps!();
        macro_rules ! registers { ($ struct_name : ident , { $ ($ name : ident = ($ val : expr , $ disp : expr)) ,+ $ (,) ? } $ (, aliases { $ ($ alias_name : ident = ($ alias_val : expr , $ alias_disp : expr)) ,+ $ (,) ? }) ?) => { # [allow (missing_docs)] impl $ struct_name { $ (pub const $ name : Register = Register ($ val) ;) + $ ($ (pub const $ alias_name : Register = Register ($ alias_val) ;) +) * } impl $ struct_name { # [doc = " The name of a register, or `None` if the register number is unknown."] # [doc = ""] # [doc = " Only returns the primary name for registers that alias with others."] pub fn register_name (register : Register) -> Option <&'static str > { match register { $ (Self ::$ name => Some ($ disp) ,) + _ => return None , } } # [doc = " Converts a register name into a register number."] pub fn name_to_register (value : & str) -> Option < Register > { match value { $ ($ disp => Some (Self ::$ name) ,) + $ ($ ($ alias_disp => Some (Self ::$ alias_name) ,) +) * _ => return None , } } } } ; }
    };
}

registers!()