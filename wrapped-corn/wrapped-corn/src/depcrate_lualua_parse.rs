// Generated macro for lua_parse (function)
macro_rules! Depcrate_lualua_parse {
() => {
// Module: crate::lua
// Provides: {"lua_parse"}
// Dependencies: {}
fn lua_parse (lua : & Lua , config : String) -> LuaResult < LuaValue > { let res = crate :: parse (& config) ; match res { Ok (v) => Ok (lua . to_value (& v) ?) , Err (e) => Err (LuaError :: RuntimeError (e . to_string ())) , } }
};
}
