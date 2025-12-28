macro_rules! deps {
    () => {
        ConfigurableFormat!();
        Formatter!();
        TimestampPrecision!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl ConfigurableFormat { # [doc = " Whether or not to write the level in the default format."] pub fn level (& mut self , write : bool) -> & mut Self { self . level = write ; self } # [doc = " Whether or not to write the source file path in the default format."] pub fn file (& mut self , write : bool) -> & mut Self { self . source_file = write ; self } # [doc = " Whether or not to write the source line number path in the default format."] # [doc = ""] # [doc = " Only has effect if `format_file` is also enabled"] pub fn line_number (& mut self , write : bool) -> & mut Self { self . source_line_number = write ; self } # [doc = " Whether or not to write the module path in the default format."] pub fn module_path (& mut self , write : bool) -> & mut Self { self . module_path = write ; self } # [doc = " Whether or not to write the target in the default format."] pub fn target (& mut self , write : bool) -> & mut Self { self . target = write ; self } # [doc = " Configures the amount of spaces to use to indent multiline log records."] # [doc = " A value of `None` disables any kind of indentation."] pub fn indent (& mut self , indent : Option < usize >) -> & mut Self { self . indent = indent ; self } # [doc = " Configures if timestamp should be included and in what precision."] pub fn timestamp (& mut self , timestamp : Option < TimestampPrecision >) -> & mut Self { self . timestamp = timestamp ; self } # [doc = " Configures the end of line suffix."] pub fn suffix (& mut self , suffix : & 'static str) -> & mut Self { self . suffix = suffix ; self } # [doc = " Set the format for structured key/value pairs in the log record"] # [doc = ""] # [doc = " With the default format, this function is called for each record and should format"] # [doc = " the structured key-value pairs as returned by [`log::Record::key_values`]."] # [doc = ""] # [doc = " The format function is expected to output the string directly to the `Formatter` so that"] # [doc = " implementations can use the [`std::fmt`] macros, similar to the main format function."] # [doc = ""] # [doc = " The default format uses a space to separate each key-value pair, with an \"=\" between"] # [doc = " the key and value."] # [cfg (feature = "kv")] pub fn key_values < F > (& mut self , format : F) -> & mut Self where F : Fn (& mut Formatter , & dyn log :: kv :: Source) -> io :: Result < () > + Sync + Send + 'static , { self . kv_format = Some (Box :: new (format)) ; self } }
    };
}

impl_78!();