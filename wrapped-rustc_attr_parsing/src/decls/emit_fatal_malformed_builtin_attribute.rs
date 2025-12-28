macro_rules! emit_fatal_malformed_builtin_attribute {
    () => {
        pub fn emit_fatal_malformed_builtin_attribute (psess : & ParseSess , attr : & Attribute , name : Symbol ,) -> ! { let template = BUILTIN_ATTRIBUTE_MAP . get (& name) . expect ("builtin attr defined") . template ; emit_malformed_attribute (psess , attr . style , attr . span , name , template) ; FatalError . raise () }
    };
}

emit_fatal_malformed_builtin_attribute!()