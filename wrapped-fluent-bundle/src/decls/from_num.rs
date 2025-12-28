macro_rules! deps {
    () => {
        FluentNumberOptions!();
        FluentValue!();
        FluentNumber!();
    };
}

macro_rules! from_num {
    () => {
        deps!();
        macro_rules ! from_num { ($ num : ty) => { impl From <$ num > for FluentNumber { fn from (n : $ num) -> Self { Self { value : n as f64 , options : FluentNumberOptions :: default () , } } } impl From <&$ num > for FluentNumber { fn from (n : &$ num) -> Self { Self { value : * n as f64 , options : FluentNumberOptions :: default () , } } } impl From < FluentNumber > for $ num { fn from (input : FluentNumber) -> Self { input . value as $ num } } impl From <& FluentNumber > for $ num { fn from (input : & FluentNumber) -> Self { input . value as $ num } } impl From <$ num > for FluentValue <'_ > { fn from (n : $ num) -> Self { FluentValue :: Number (n . into ()) } } impl From <&$ num > for FluentValue <'_ > { fn from (n : &$ num) -> Self { FluentValue :: Number (n . into ()) } } } ; ($ ($ num : ty) +) => { $ (from_num ! ($ num) ;) + } ; }
    };
}

from_num!()