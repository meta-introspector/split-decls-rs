macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_820 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> result :: Result < () , fmt :: Error > { match * self { Error :: OffsetOutOfBounds => write ! (f , "The given offset is out of bounds.") , Error :: LengthOutOfBounds => write ! (f , "The given length is out of bounds.") , Error :: InvalidAttributeValue => { write ! (f , "The attribute value is an invalid for writing.") } Error :: ValueTooLarge => write ! (f , "The value is too large for the encoding form.") , Error :: UnsupportedWordSize (size) => write ! (f , "Unsupported word size: {}" , size) , Error :: UnsupportedVersion (version) => { write ! (f , "Unsupported DWARF version: {}" , version) } Error :: InitialLengthOverflow => write ! (f , "The unit length is too large for the requested DWARF format.") , Error :: InvalidAddress => write ! (f , "The address is invalid.") , Error :: InvalidReference => write ! (f , "The reference is invalid.") , Error :: NeedVersion (version) => write ! (f , "A requested feature requires a DWARF version {}." , version) , Error :: LineStringFormMismatch => { write ! (f , "Strings in line number program have mismatched forms.") } Error :: InvalidRange => write ! (f , "The range is empty or otherwise invalid.") , Error :: IncompatibleLineProgramEncoding => write ! (f , "The line number program encoding is incompatible with the unit encoding.") , Error :: InvalidFrameCodeOffset (offset) => write ! (f , "Could not encode code offset ({}) for a frame instruction." , offset ,) , Error :: InvalidFrameDataOffset (offset) => write ! (f , "Could not encode data offset ({}) for a frame instruction." , offset ,) , Error :: UnsupportedPointerEncoding (eh_pe) => { write ! (f , "Unsupported eh_frame pointer encoding ({})." , eh_pe) } Error :: UnsupportedCfiExpressionReference => { write ! (f , "Unsupported reference in CFI expression.") } Error :: UnsupportedExpressionForwardReference => { write ! (f , "Unsupported forward reference in expression.") } } } }
    };
}

impl_820!()