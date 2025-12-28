macro_rules! deps {
    () => {
        CliReport!();
        CliVerbosity!();
        LabeledSample!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl CliReport { pub fn new (enable_text_overwrite : bool , enable_text_coloring : bool , verbosity : CliVerbosity ,) -> CliReport { CliReport { enable_text_overwrite , enable_text_coloring , verbosity , } } fn text_overwrite (& self) { if self . enable_text_overwrite { eprint ! ("\r{}" , ClearLine :: All) ; } } # [allow (clippy :: needless_pass_by_value)] fn print_overwritable (& self , s : String) { if self . enable_text_overwrite { eprint ! ("{}" , s) ; stderr () . flush () . unwrap () ; } else { eprintln ! ("{}" , s) ; } } fn with_color (& self , color : Color , s : & str) -> String { if self . enable_text_coloring { format ! ("{}{}{}" , SetForegroundColor (color) , s , ResetAttributes) } else { String :: from (s) } } fn green (& self , s : & str) -> String { self . with_color (Color :: DarkGreen , s) } fn yellow (& self , s : & str) -> String { self . with_color (Color :: DarkYellow , s) } fn red (& self , s : & str) -> String { self . with_color (Color :: DarkRed , s) } fn bold (& self , s : String) -> String { if self . enable_text_coloring { format ! ("{}{}{}" , SetAttribute (Attribute :: Bold) , s , ResetAttributes) } else { s } } fn faint (& self , s : String) -> String { if self . enable_text_coloring { format ! ("{}{}{}" , SetAttribute (Attribute :: Faint) , s , ResetAttributes) } else { s } } pub fn outliers (& self , sample : & LabeledSample < '_ , f64 >) { let (los , lom , _ , him , his) = sample . count () ; let noutliers = los + lom + him + his ; let sample_size = sample . len () ; if noutliers == 0 { return ; } let percent = | n : usize | 100. * n as f64 / sample_size as f64 ; println ! ("{}" , self . yellow (& format ! ("Found {} outliers among {} measurements ({:.2}%)" , noutliers , sample_size , percent (noutliers)))) ; let print = | n , label | { if n != 0 { println ! ("  {} ({:.2}%) {}" , n , percent (n) , label) ; } } ; print (los , "low severe") ; print (lom , "low mild") ; print (him , "high mild") ; print (his , "high severe") ; } }
    };
}

impl_279!();