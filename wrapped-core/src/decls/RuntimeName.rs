macro_rules! RuntimeName {
    () => {
        # [doc (hidden)] pub trait RuntimeName { const NAME : & 'static str = "" ; }
    };
}

RuntimeName!()