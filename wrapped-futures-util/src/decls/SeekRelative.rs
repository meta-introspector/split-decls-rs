macro_rules! SeekRelative {
    () => {
        # [doc = " Future for the [`BufReader::seek_relative`](self::BufReader::seek_relative) method."] # [derive (Debug)] # [must_use = "futures do nothing unless polled"] pub struct SeekRelative < 'a , R > { inner : Pin < & 'a mut BufReader < R > > , offset : i64 , first : bool , }
    };
}

SeekRelative!()