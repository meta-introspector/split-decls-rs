macro_rules! deps {
    () => {
        Features!();
    };
}

macro_rules! AttributeGate {
    () => {
        deps!();
        # [derive (Clone , Debug , Copy)] pub enum AttributeGate { # [doc = " A gated attribute which requires a feature gate to be enabled."] Gated { # [doc = " The feature gate, for example `#![feature(rustc_attrs)]` for rustc_* attributes."] feature : Symbol , # [doc = " The error message displayed when an attempt is made to use the attribute without its feature gate."] message : & 'static str , # [doc = " Check function to be called during the `PostExpansionVisitor` pass."] check : fn (& Features) -> bool , # [doc = " Notes to be displayed when an attempt is made to use the attribute without its feature gate."] notes : & 'static [& 'static str] , } , # [doc = " Ungated attribute, can be used on all release channels"] Ungated , }
    };
}

AttributeGate!();