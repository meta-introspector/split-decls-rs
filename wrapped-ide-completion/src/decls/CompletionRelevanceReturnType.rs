macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! CompletionRelevanceReturnType {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum CompletionRelevanceReturnType { Other , # [doc = " Returns the Self type of the impl/trait"] DirectConstructor , # [doc = " Returns something that indirectly constructs the `Self` type of the impl/trait e.g. `Result<Self, ()>`, `Option<Self>`"] Constructor , # [doc = " Returns a possible builder for the type"] Builder , }
    };
}

CompletionRelevanceReturnType!()