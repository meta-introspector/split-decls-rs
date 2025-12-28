macro_rules! deps {
    () => {
        EntryMarker!();
    };
}

macro_rules! private {
    () => {
        deps!();
        pub (crate) mod private { # [doc = " Private trait that we use as a supertrait of `EntryMarker` to prevent it from being"] # [doc = " implemented from outside this crate."] # [doc = ""] # [doc = " See this [blog](https://jack.wrenn.fyi/blog/private-trait-methods/) for more details."] pub trait Sealed { } }
    };
}

private!();