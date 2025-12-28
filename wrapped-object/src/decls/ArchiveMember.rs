macro_rules! deps {
    () => {
        MemberHeader!();
    };
}

macro_rules! ArchiveMember {
    () => {
        deps!();
        # [doc = " A partially parsed archive member."] # [derive (Debug)] pub struct ArchiveMember < 'data > { header : MemberHeader < 'data > , name : & 'data [u8] , offset : u64 , size : u64 , }
    };
}

ArchiveMember!()