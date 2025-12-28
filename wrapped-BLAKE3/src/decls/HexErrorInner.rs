macro_rules! HexErrorInner {
    () => {
        # [derive (Clone , Debug)] enum HexErrorInner { InvalidByte (u8) , InvalidLen (usize) , }
    };
}

HexErrorInner!();