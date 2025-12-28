macro_rules! deps {
    () => {
        FormatArgument!();
        FormatArgumentKind!();
        FormatArguments!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl FormatArguments { pub fn new () -> Self { Self { arguments : Vec :: new () , names : FxHashMap :: default () , num_unnamed_args : 0 , num_explicit_args : 0 , } } pub fn add (& mut self , arg : FormatArgument) -> usize { let index = self . arguments . len () ; if let Some (name) = arg . kind . ident () { self . names . insert (name . name , index) ; } else if self . names . is_empty () { self . num_unnamed_args += 1 ; } if ! matches ! (arg . kind , FormatArgumentKind :: Captured (..)) { assert_eq ! (self . num_explicit_args , self . arguments . len () , "captured arguments must be added last") ; self . num_explicit_args += 1 ; } self . arguments . push (arg) ; index } pub fn by_name (& self , name : Symbol) -> Option < (usize , & FormatArgument) > { let i = * self . names . get (& name) ? ; Some ((i , & self . arguments [i])) } pub fn by_index (& self , i : usize) -> Option < & FormatArgument > { (i < self . num_explicit_args) . then (| | & self . arguments [i]) } pub fn unnamed_args (& self) -> & [FormatArgument] { & self . arguments [.. self . num_unnamed_args] } pub fn named_args (& self) -> & [FormatArgument] { & self . arguments [self . num_unnamed_args .. self . num_explicit_args] } pub fn explicit_args (& self) -> & [FormatArgument] { & self . arguments [.. self . num_explicit_args] } pub fn all_args (& self) -> & [FormatArgument] { & self . arguments [..] } pub fn all_args_mut (& mut self) -> & mut Vec < FormatArgument > { & mut self . arguments } }
    };
}

impl_339!()