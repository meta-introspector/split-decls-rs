// Generated macro for libcorn (function)
macro_rules! Depcrate_lualibcorn {
() => {
// Module: crate::lua
// Provides: {"libcorn"}
// Dependencies: {}
# [mlua :: lua_module] fn libcorn (lua : & Lua) -> LuaResult < LuaTable > { let exports = lua . create_table () ? ; let parse = lua . create_function (lua_parse) ? ; exports . set ("parse" , parse) ? ; Ok (exports) }
};
}
