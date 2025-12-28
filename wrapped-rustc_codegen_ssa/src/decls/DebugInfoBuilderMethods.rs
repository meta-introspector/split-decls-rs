macro_rules! deps {
    () => {
        BackendTypes!();
    };
}

macro_rules! DebugInfoBuilderMethods {
    () => {
        deps!();
        pub trait DebugInfoBuilderMethods : BackendTypes { fn dbg_var_addr (& mut self , dbg_var : Self :: DIVariable , dbg_loc : Self :: DILocation , variable_alloca : Self :: Value , direct_offset : Size , indirect_offsets : & [Size] , fragment : Option < Range < Size > > ,) ; fn set_dbg_loc (& mut self , dbg_loc : Self :: DILocation) ; fn clear_dbg_loc (& mut self) ; fn insert_reference_to_gdb_debug_scripts_section_global (& mut self) ; fn set_var_name (& mut self , value : Self :: Value , name : & str) ; }
    };
}

DebugInfoBuilderMethods!();