macro_rules! RTLD_LOCAL {
    () => {
        # [doc = " Load symbols into an isolated namespace."] # [doc = ""] # [doc = " The executable object file's symbols shall not be made available for relocation processing of"] # [doc = " any other executable object file. This mode of operation is most appropriate for e.g. plugins."] pub const RTLD_LOCAL : c_int = posix :: RTLD_LOCAL ;
    };
}

RTLD_LOCAL!()