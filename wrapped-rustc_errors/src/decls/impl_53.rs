macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Level { fn color (self) -> ColorSpec { let mut spec = ColorSpec :: new () ; match self { Bug | Fatal | Error | DelayedBug => { spec . set_fg (Some (Color :: Red)) . set_intense (true) ; } ForceWarning | Warning => { spec . set_fg (Some (Color :: Yellow)) . set_intense (cfg ! (windows)) ; } Note | OnceNote => { spec . set_fg (Some (Color :: Green)) . set_intense (true) ; } Help | OnceHelp => { spec . set_fg (Some (Color :: Cyan)) . set_intense (true) ; } FailureNote => { } Allow | Expect => unreachable ! () , } spec } pub fn to_str (self) -> & 'static str { match self { Bug | DelayedBug => "error: internal compiler error" , Fatal | Error => "error" , ForceWarning | Warning => "warning" , Note | OnceNote => "note" , Help | OnceHelp => "help" , FailureNote => "failure-note" , Allow | Expect => unreachable ! () , } } pub fn is_failure_note (& self) -> bool { matches ! (* self , FailureNote) } fn can_be_subdiag (& self) -> bool { match self { Bug | DelayedBug | Fatal | Error | ForceWarning | FailureNote | Allow | Expect => false , Warning | Note | Help | OnceNote | OnceHelp => true , } } }
    };
}

impl_53!()