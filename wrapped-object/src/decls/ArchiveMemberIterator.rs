macro_rules! deps {
    () => {
        ReadRef!();
        Members!();
    };
}

macro_rules! ArchiveMemberIterator {
    () => {
        deps!();
        # [doc = " An iterator over the members of an archive."] # [derive (Debug)] pub struct ArchiveMemberIterator < 'data , R : ReadRef < 'data > = & 'data [u8] > { data : R , members : Members < 'data > , names : & 'data [u8] , thin : bool , }
    };
}

ArchiveMemberIterator!()