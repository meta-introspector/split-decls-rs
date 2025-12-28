macro_rules! deps {
    () => {
        MessageCopyState!();
        MessageRingBuffer!();
        Message!();
        MessageLevel!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl MessageRingBuffer { # [doc = " Create a new instance the ability to hold `capacity` amount of messages."] pub fn with_capacity (capacity : usize) -> MessageRingBuffer { MessageRingBuffer { buf : Vec :: with_capacity (capacity) , cursor : 0 , total : 0 , } } # [doc = " Push a `message` from `origin` at severity `level` into the buffer, possibly overwriting the last message added."] pub fn push_overwrite (& mut self , level : MessageLevel , origin : String , message : impl Into < String >) { let msg = Message { time : SystemTime :: now () , level , origin , message : message . into () , } ; if self . has_capacity () { self . buf . push (msg) } else { self . buf [self . cursor] = msg ; self . cursor = (self . cursor + 1) % self . buf . len () ; } self . total = self . total . wrapping_add (1) ; } # [doc = " Copy all messages currently contained in the buffer to `out`."] pub fn copy_all (& self , out : & mut Vec < Message >) { out . clear () ; if self . buf . is_empty () { return ; } out . extend_from_slice (& self . buf [self . cursor % self . buf . len () ..]) ; if self . cursor != self . buf . len () { out . extend_from_slice (& self . buf [.. self . cursor]) ; } } # [doc = " Copy all new messages into `out` that where received since the last time this method was called provided"] # [doc = " its `previous` return value."] pub fn copy_new (& self , out : & mut Vec < Message > , previous : Option < MessageCopyState >) -> MessageCopyState { out . clear () ; match previous { Some (MessageCopyState { cursor , buf_len , total }) => { if self . total . saturating_sub (total) >= self . buf . capacity () { self . copy_all (out) ; } else { let new_elements_below_cap = self . buf . len () . saturating_sub (buf_len) ; let cursor_ofs : isize = self . cursor as isize - cursor as isize ; match cursor_ofs { 0 => { out . extend_from_slice (& self . buf [self . buf . len () - new_elements_below_cap ..]) ; } c if c > 0 => { out . extend_from_slice (& self . buf [(cursor % self . buf . len ()) .. self . cursor]) ; } c if c < 0 => { out . extend_from_slice (& self . buf [(cursor % self . buf . len ()) ..]) ; out . extend_from_slice (& self . buf [.. self . cursor]) ; } _ => unreachable ! ("logic dictates that… yeah, you really shouldn't ever see this!") , } } } None => self . copy_all (out) , } ; MessageCopyState { cursor : self . cursor , buf_len : self . buf . len () , total : self . total , } } fn has_capacity (& self) -> bool { self . buf . len () < self . buf . capacity () } }
    };
}

impl_140!();