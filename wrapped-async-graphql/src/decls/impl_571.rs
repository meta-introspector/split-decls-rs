macro_rules! deps {
    () => {
        Data!();
        Any!();
        Error!();
        Result!();
        ExtensionContext!();
        Schema!();
        Query!();
        Context!();
        SDLExportOptions!();
    };
}

macro_rules! impl_571 {
    () => {
        deps!();
        impl < 'a > ExtensionContext < 'a > { # [doc = " Convert the specified [ExecutableDocument] into a query string."] # [doc = ""] # [doc = " Usually used for log extension, it can hide secret arguments."] pub fn stringify_execute_doc (& self , doc : & ExecutableDocument , variables : & Variables) -> String { self . schema_env . registry . stringify_exec_doc (variables , doc) . unwrap_or_default () } # [doc = " Returns SDL(Schema Definition Language) of this schema."] pub fn sdl (& self) -> String { self . schema_env . registry . export_sdl (Default :: default ()) } # [doc = " Returns SDL(Schema Definition Language) of this schema with options."] pub fn sdl_with_options (& self , options : SDLExportOptions) -> String { self . schema_env . registry . export_sdl (options) } # [doc = " Gets the global data defined in the `Context` or `Schema`."] # [doc = ""] # [doc = " If both `Schema` and `Query` have the same data type, the data in the"] # [doc = " `Query` is obtained."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns a `Error` if the specified type data does not exist."] pub fn data < D : Any + Send + Sync > (& self) -> Result < & 'a D > { self . data_opt :: < D > () . ok_or_else (| | { Error :: new (format ! ("Data `{}` does not exist." , std :: any :: type_name ::< D > ())) }) } # [doc = " Gets the global data defined in the `Context` or `Schema`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " It will panic if the specified data type does not exist."] pub fn data_unchecked < D : Any + Send + Sync > (& self) -> & 'a D { self . data_opt :: < D > () . unwrap_or_else (| | panic ! ("Data `{}` does not exist." , std :: any :: type_name ::< D > ())) } # [doc = " Gets the global data defined in the `Context` or `Schema` or `None` if"] # [doc = " the specified type data does not exist."] pub fn data_opt < D : Any + Send + Sync > (& self) -> Option < & 'a D > { self . query_data . and_then (| query_data | query_data . get (& TypeId :: of :: < D > ())) . or_else (| | self . session_data . get (& TypeId :: of :: < D > ())) . or_else (| | self . schema_env . data . get (& TypeId :: of :: < D > ())) . and_then (| d | d . downcast_ref :: < D > ()) } }
    };
}

impl_571!()