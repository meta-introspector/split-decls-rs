macro_rules! PanicBuilder {
    () => {
        pub (super) struct PanicBuilder { msg : String , locations : Vec < (String , Option < usize > , Location) > , }
    };
}

PanicBuilder!();