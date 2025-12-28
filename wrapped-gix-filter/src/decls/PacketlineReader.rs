macro_rules! PacketlineReader {
    () => {
        type PacketlineReader < 'a , T = std :: process :: ChildStdout > = WithSidebands < 'a , T , fn (bool , & [u8]) -> gix_packetline :: read :: ProgressAction > ;
    };
}

PacketlineReader!();