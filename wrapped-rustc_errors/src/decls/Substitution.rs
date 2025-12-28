macro_rules! deps {
    () => {
        SubstitutionPart!();
    };
}

macro_rules! Substitution {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Hash , Encodable , Decodable)] # [doc = " See the docs on `CodeSuggestion::substitutions`"] pub struct Substitution { pub parts : Vec < SubstitutionPart > , }
    };
}

Substitution!()