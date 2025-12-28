macro_rules! Conversion {
    () => {
        # [derive (Copy , Clone)] enum Conversion { Into , AsRef , AsMut , }
    };
}

Conversion!();