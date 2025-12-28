macro_rules! Specified {
    () => {
        # [doc = " Indicates wether a colored is specified by the prefix \"fg:\" or \"bg:\"."] # [derive (Debug , Clone , Copy)] enum Specified { True , False , }
    };
}

Specified!()