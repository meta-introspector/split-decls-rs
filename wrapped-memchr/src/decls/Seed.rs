macro_rules! Seed {
    () => {
        # [doc = " A single substring test for forward and reverse searches."] # [doc = ""] # [doc = " Each seed is valid on its own, but it also serves as a starting point"] # [doc = " to generate more tests. Namely, we pad out the haystacks with other"] # [doc = " characters so that we get more complete coverage. This is especially useful"] # [doc = " for testing vector algorithms that tend to have weird special cases for"] # [doc = " alignment and loop unrolling."] # [doc = ""] # [doc = " Padding works by assuming certain characters never otherwise appear in a"] # [doc = " needle or a haystack. Neither should contain a `#` character."] # [derive (Clone , Copy , Debug)] struct Seed { needle : & 'static str , haystack : & 'static str , fwd : Option < usize > , rev : Option < usize > , }
    };
}

Seed!();