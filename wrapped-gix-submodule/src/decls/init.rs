macro_rules! deps {
    () => {
        File!();
        Error!();
    };
}

macro_rules! init {
    () => {
        deps!();
        # [doc = ""] mod init { use std :: path :: PathBuf ; use crate :: File ; impl std :: fmt :: Debug for File { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("File") . field ("config_path" , & self . config_path ()) . field ("config" , & format_args ! ("r#\"{}\"#" , self . config)) . finish () } } # [doc = " A marker we use when listing names to not pick them up from overridden sections."] pub (crate) const META_MARKER : gix_config :: Source = gix_config :: Source :: Api ; # [doc = " Lifecycle"] impl File { # [doc = " Parse `bytes` as git configuration, typically from `.gitmodules`, without doing any further validation."] # [doc = " `path` can be provided to keep track of where the file was read from in the underlying [`config`](Self::config())"] # [doc = " instance."] # [doc = " `config` is used to [apply value overrides](File::append_submodule_overrides), which can be empty if overrides"] # [doc = " should be applied at a later time."] # [doc = ""] # [doc = " Future access to the module information is lazy and configuration errors are exposed there on a per-value basis."] # [doc = ""] # [doc = " ### Security Considerations"] # [doc = ""] # [doc = " The information itself should be used with care as it can direct the caller to fetch from remotes. It is, however,"] # [doc = " on the caller to assure the input data can be trusted."] pub fn from_bytes (bytes : & [u8] , path : impl Into < Option < PathBuf > > , config : & gix_config :: File < '_ > ,) -> Result < Self , gix_config :: parse :: Error > { let metadata = { let mut meta = gix_config :: file :: Metadata :: from (META_MARKER) ; meta . path = path . into () ; meta } ; let modules = gix_config :: File :: from_parse_events_no_includes (gix_config :: parse :: Events :: from_bytes_owned (bytes , None) ? , metadata ,) ; let mut res = Self { config : modules } ; res . append_submodule_overrides (config) ; Ok (res) } # [doc = " Turn ourselves into the underlying parsed configuration file."] pub fn into_config (self) -> gix_config :: File < 'static > { self . config } } }
    };
}

init!()