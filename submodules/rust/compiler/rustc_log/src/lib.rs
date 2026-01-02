mkuse!{use std :: env :: { self , VarError } ;}
mkuse!{use std :: fmt :: { self , Display } ;}
mkuse!{use std :: io :: { self , IsTerminal } ;}
mkuse!{use tracing :: dispatcher :: SetGlobalDefaultError ;}
mkuse!{use tracing :: { Event , Subscriber } ;}
mkuse!{use tracing_subscriber :: filter :: { Directive , EnvFilter , LevelFilter } ;}
mkuse!{use tracing_subscriber :: fmt :: FmtContext ;}
mkuse!{use tracing_subscriber :: fmt :: format :: { self , FormatEvent , FormatFields } ;}
mkuse!{use tracing_subscriber :: layer :: SubscriberExt ;}
mkuse!{use tracing_subscriber :: { Layer , Registry } ;}
mkitem!{mkstruct!{# [doc = " The values of all the environment variables that matter for configuring a logger."] # [doc = " Errors are explicitly preserved so that we can share error handling."] pub struct LoggerConfig { pub filter : Result < String , VarError > , pub color_logs : Result < String , VarError > , pub verbose_entry_exit : Result < String , VarError > , pub verbose_thread_ids : Result < String , VarError > , pub backtrace : Result < String , VarError > , pub wraptree : Result < String , VarError > , pub lines : Result < String , VarError > , }}}
mkitem!{mkimpl!{impl LoggerConfig { pub fn from_env (env : & str) -> Self { LoggerConfig { filter : env :: var (env) , color_logs : env :: var (format ! ("{env}_COLOR")) , verbose_entry_exit : env :: var (format ! ("{env}_ENTRY_EXIT")) , verbose_thread_ids : env :: var (format ! ("{env}_THREAD_IDS")) , backtrace : env :: var (format ! ("{env}_BACKTRACE")) , wraptree : env :: var (format ! ("{env}_WRAPTREE")) , lines : env :: var (format ! ("{env}_LINES")) , } } }}}

macro_rules! init_logger_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_logger in module {}", module_path!());
    };
}

mkfn!{
    init_logger_introspect!();
    # [doc = " Initialize the logger with the given values for the filter, coloring, and other options env variables."] pub fn init_logger (cfg : LoggerConfig) -> Result < () , Error > { init_logger_with_additional_layer (cfg , | | Registry :: default ()) }
}
mkitem!{mktrait!{# [doc = " Trait alias for the complex return type of `build_subscriber` in"] # [doc = " [init_logger_with_additional_layer]. A [Registry] with any composition of [tracing::Subscriber]s"] # [doc = " (e.g. `Registry::default().with(custom_layer)`) should be compatible with this type."] # [doc = " Having an alias is also useful so rustc_driver_impl does not need to explicitly depend on"] # [doc = " `tracing_subscriber`."] pub trait BuildSubscriberRet : tracing :: Subscriber + for < 'span > tracing_subscriber :: registry :: LookupSpan < 'span > + Send + Sync { }}}
mkitem!{mkimpl!{impl < T : tracing :: Subscriber + for < 'span > tracing_subscriber :: registry :: LookupSpan < 'span > + Send + Sync , > BuildSubscriberRet for T { }}}

macro_rules! init_logger_with_additional_layer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_logger_with_additional_layer in module {}", module_path!());
    };
}

mkfn!{
    init_logger_with_additional_layer_introspect!();
    # [doc = " Initialize the logger with the given values for the filter, coloring, and other options env variables."] # [doc = " Additionally add a custom layer to collect logging and tracing events via `build_subscriber`,"] # [doc = " for example: `|| Registry::default().with(custom_layer)`."] pub fn init_logger_with_additional_layer < F , T > (cfg : LoggerConfig , build_subscriber : F ,) -> Result < () , Error > where F : FnOnce () -> T , T : BuildSubscriberRet , { let filter = match cfg . filter { Ok (env) => EnvFilter :: new (env) , _ => EnvFilter :: default () . add_directive (Directive :: from (LevelFilter :: WARN)) , } ; let color_logs = match cfg . color_logs { Ok (value) => match value . as_ref () { "always" => true , "never" => false , "auto" => stderr_isatty () , _ => return Err (Error :: InvalidColorValue (value)) , } , Err (VarError :: NotPresent) => stderr_isatty () , Err (VarError :: NotUnicode (_value)) => return Err (Error :: NonUnicodeColorValue) , } ; let verbose_entry_exit = match cfg . verbose_entry_exit { Ok (v) => & v != "0" , Err (_) => false , } ; let verbose_thread_ids = match cfg . verbose_thread_ids { Ok (v) => & v == "1" , Err (_) => false , } ; let lines = match cfg . lines { Ok (v) => & v == "1" , Err (_) => false , } ; let mut layer = tracing_tree :: HierarchicalLayer :: default () . with_writer (io :: stderr) . with_ansi (color_logs) . with_targets (true) . with_verbose_exit (verbose_entry_exit) . with_verbose_entry (verbose_entry_exit) . with_indent_amount (2) . with_indent_lines (lines) . with_thread_ids (verbose_thread_ids) . with_thread_names (verbose_thread_ids) ; match cfg . wraptree { Ok (v) => match v . parse :: < usize > () { Ok (v) => { layer = layer . with_wraparound (v) ; } Err (_) => return Err (Error :: InvalidWraptree (v)) , } , Err (_) => { } } let subscriber = build_subscriber () . with (layer . with_filter (filter)) ; match cfg . backtrace { Ok (backtrace_target) => { let fmt_layer = tracing_subscriber :: fmt :: layer () . with_writer (io :: stderr) . without_time () . event_format (BacktraceFormatter { backtrace_target }) ; let subscriber = subscriber . with (fmt_layer) ; tracing :: subscriber :: set_global_default (subscriber) ? ; } Err (_) => { tracing :: subscriber :: set_global_default (subscriber) ? ; } } ; Ok (()) }
}
mkitem!{mkstruct!{struct BacktraceFormatter { backtrace_target : String , }}}
mkitem!{mkimpl!{impl < S , N > FormatEvent < S , N > for BacktraceFormatter where S : Subscriber + for < 'a > tracing_subscriber :: registry :: LookupSpan < 'a > , N : for < 'a > FormatFields < 'a > + 'static , { fn format_event (& self , _ctx : & FmtContext < '_ , S , N > , mut writer : format :: Writer < '_ > , event : & Event < '_ > ,) -> fmt :: Result { let target = event . metadata () . target () ; if ! target . contains (& self . backtrace_target) { return Ok (()) ; } let backtrace = std :: backtrace :: Backtrace :: force_capture () ; writeln ! (writer , "stack backtrace: \n{backtrace:?}") } }}}

macro_rules! stdout_isatty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stdout_isatty in module {}", module_path!());
    };
}

mkfn!{
    stdout_isatty_introspect!();
    pub fn stdout_isatty () -> bool { io :: stdout () . is_terminal () }
}

macro_rules! stderr_isatty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_isatty in module {}", module_path!());
    };
}

mkfn!{
    stderr_isatty_introspect!();
    pub fn stderr_isatty () -> bool { io :: stderr () . is_terminal () }
}
mkitem!{mkenum!{# [derive (Debug)] pub enum Error { InvalidColorValue (String) , NonUnicodeColorValue , InvalidWraptree (String) , AlreadyInit (SetGlobalDefaultError) , }}}
mkitem!{mkimpl!{impl std :: error :: Error for Error { }}}
mkitem!{mkimpl!{impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: InvalidColorValue (value) => write ! (formatter , "invalid log color value '{value}': expected one of always, never, or auto" ,) , Error :: NonUnicodeColorValue => write ! (formatter , "non-Unicode log color value: expected one of always, never, or auto" ,) , Error :: InvalidWraptree (value) => write ! (formatter , "invalid log WRAPTREE value '{value}': expected a non-negative integer" ,) , Error :: AlreadyInit (tracing_error) => Display :: fmt (tracing_error , formatter) , } } }}}
mkitem!{mkimpl!{impl From < SetGlobalDefaultError > for Error { fn from (tracing_error : SetGlobalDefaultError) -> Self { Error :: AlreadyInit (tracing_error) } }}}