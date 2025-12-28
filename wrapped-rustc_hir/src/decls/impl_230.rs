macro_rules! deps {
    () => {
        MatchSource!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl MatchSource { # [inline] pub const fn name (self) -> & 'static str { use MatchSource :: * ; match self { Normal => "match" , Postfix => ".match" , ForLoopDesugar => "for" , TryDesugar (_) => "?" , AwaitDesugar => ".await" , FormatArgs => "format_args!()" , } } }
    };
}

impl_230!()