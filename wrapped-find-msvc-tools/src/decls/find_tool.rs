macro_rules! deps {
    () => {
        Tool!();
        StdEnvGetter!();
    };
}

macro_rules! find_tool {
    () => {
        deps!();
        # [doc = " Similar to the `find` function above, this function will attempt the same"] # [doc = " operation (finding a MSVC tool in a local install) but instead returns a"] # [doc = " `Tool` which may be introspected."] pub fn find_tool (arch_or_target : & str , tool : & str) -> Option < Tool > { let full_arch = if let Some ((full_arch , rest)) = arch_or_target . split_once ("-") { if ! rest . contains ("msvc") { return None ; } full_arch } else { arch_or_target } ; find_tool_with_env (full_arch , tool , & StdEnvGetter) }
    };
}

find_tool!()