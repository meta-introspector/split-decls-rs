macro_rules! deps {
    () => {
        State!();
        TagRefIter!();
        Error!();
        Token!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a > TagRefIter < 'a > { # [doc = " Create a tag iterator from data."] pub fn from_bytes (data : & 'a [u8]) -> TagRefIter < 'a > { TagRefIter { data , state : State :: default () , } } # [doc = " Returns the target id of this tag if it is the first function called and if there is no error in decoding"] # [doc = " the data."] # [doc = ""] # [doc = " Note that this method must only be called once or else will always return None while consuming a single token."] # [doc = " Errors are coerced into options, hiding whether there was an error or not. The caller should assume an error if they"] # [doc = " call the method as intended. Such a squelched error cannot be recovered unless the objects data is retrieved and parsed again."] # [doc = " `next()`."] pub fn target_id (mut self) -> Result < ObjectId , crate :: decode :: Error > { let token = self . next () . ok_or_else (missing_field) ? ? ; Token :: into_id (token) . ok_or_else (missing_field) } # [doc = " Returns the taggers signature if there is no decoding error, and if this field exists."] # [doc = " Errors are coerced into options, hiding whether there was an error or not. The caller knows if there was an error or not."] pub fn tagger (mut self) -> Result < Option < gix_actor :: SignatureRef < 'a > > , crate :: decode :: Error > { self . find_map (| t | match t { Ok (Token :: Tagger (signature)) => Some (Ok (signature)) , Err (err) => Some (Err (err)) , _ => None , }) . ok_or_else (missing_field) ? } }
    };
}

impl_93!()