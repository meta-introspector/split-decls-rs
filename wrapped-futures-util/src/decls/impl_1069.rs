macro_rules! deps {
    () => {
        Current!();
        SeekRelative!();
    };
}

macro_rules! impl_1069 {
    () => {
        deps!();
        impl < R > Future for SeekRelative < '_ , R > where R : AsyncRead + AsyncSeek , { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let offset = self . offset ; if self . first { self . first = false ; self . inner . as_mut () . poll_seek_relative (cx , offset) } else { self . inner . as_mut () . as_mut () . poll_seek (cx , SeekFrom :: Current (offset)) . map (| res | res . map (| _ | ())) } } }
    };
}

impl_1069!()