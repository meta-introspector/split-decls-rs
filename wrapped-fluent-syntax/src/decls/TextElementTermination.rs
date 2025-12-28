macro_rules! TextElementTermination {
    () => {
        # [derive (Debug , PartialEq)] enum TextElementTermination { LineFeed , Crlf , PlaceableStart , Eof , }
    };
}

TextElementTermination!();