macro_rules! deps {
    () => {
        StreamKind!();
        Options!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [doc = " Convenience"] impl Options { # [doc = " Automatically configure (and overwrite) the following fields based on terminal configuration."] # [doc = ""] # [doc = " * output_is_terminal"] # [doc = " * colored"] # [doc = " * terminal_dimensions"] # [doc = " * hide-cursor (based on presence of 'signal-hook' feature."] # [cfg (feature = "render-line-autoconfigure")] pub fn auto_configure (mut self , output : StreamKind) -> Self { self . output_is_terminal = match output { StreamKind :: Stdout => is_terminal :: is_terminal (std :: io :: stdout ()) , StreamKind :: Stderr => is_terminal :: is_terminal (std :: io :: stderr ()) , } ; self . colored = self . output_is_terminal && crosstermion :: color :: allowed () ; self . terminal_dimensions = crosstermion :: terminal :: size () . unwrap_or ((80 , 20)) ; # [cfg (feature = "signal-hook")] self . auto_hide_cursor () ; self } # [cfg (all (feature = "render-line-autoconfigure" , feature = "signal-hook"))] fn auto_hide_cursor (& mut self) { self . hide_cursor = true ; } # [cfg (not (feature = "render-line-autoconfigure"))] # [doc = " No-op - only available with the `render-line-autoconfigure` feature toggle."] pub fn auto_configure (self , _output : StreamKind) -> Self { self } }
    };
}

impl_81!();