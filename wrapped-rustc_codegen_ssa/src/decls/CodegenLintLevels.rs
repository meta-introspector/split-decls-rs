macro_rules! CodegenLintLevels {
    () => {
        # [doc = " A list of lint levels used in codegen."] # [doc = ""] # [doc = " When using `-Z link-only`, we don't have access to the tcx and must work"] # [doc = " solely from the `.rlink` file. `Lint`s are defined too early to be encodeable."] # [doc = " Instead, encode exactly the information we need."] # [derive (Copy , Clone , Debug , Encodable , Decodable)] pub struct CodegenLintLevels { linker_messages : LevelAndSource , }
    };
}

CodegenLintLevels!()