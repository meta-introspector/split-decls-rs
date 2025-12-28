macro_rules! StringBinaryProperty {
    () => {
        # [doc = " This type can represent any binary property over strings."] # [doc = ""] # [doc = " This is intended to be used in situations where the exact unicode property needed is"] # [doc = " only known at runtime, for example in regex engines."] # [doc = ""] # [doc = " The values are intended to be identical to ICU4C's UProperty enum"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [allow (dead_code)] # [allow (missing_docs)] enum StringBinaryProperty { BasicEmoji = 65 , EmojiKeycapSequence = 66 , RgiEmoji = 71 , RgiEmojiFlagSequence = 68 , RgiEmojiModifierSequence = 67 , RgiEmojiTagSequence = 69 , RgiEmojiZWJSequence = 70 , }
    };
}

StringBinaryProperty!();