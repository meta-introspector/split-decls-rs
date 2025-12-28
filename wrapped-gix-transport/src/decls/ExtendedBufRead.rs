macro_rules! deps {
    () => {
        MessageKind!();
        Protocol!();
        Error!();
        HandleProgress!();
        ReadlineBufRead!();
    };
}

macro_rules! ExtendedBufRead {
    () => {
        deps!();
        # [doc = " Provide even more access to the underlying packet reader."] pub trait ExtendedBufRead < 'a > : ReadlineBufRead { # [doc = " Set the handler to which progress will be delivered."] # [doc = ""] # [doc = " Note that this is only possible if packet lines are sent in side band mode."] fn set_progress_handler (& mut self , handle_progress : Option < HandleProgress < 'a > >) ; # [doc = " Peek the next data packet line. Maybe None if the next line is a packet we stop at, queryable using"] # [doc = " [`stopped_at()`][ExtendedBufRead::stopped_at()]."] fn peek_data_line (& mut self) -> Option < io :: Result < Result < & [u8] , Error > > > ; # [doc = " Resets the reader to allow reading past a previous stop, and sets delimiters according to the"] # [doc = " given protocol."] fn reset (& mut self , version : Protocol) ; # [doc = " Return the kind of message at which the reader stopped."] fn stopped_at (& self) -> Option < MessageKind > ; }
    };
}

ExtendedBufRead!();