macro_rules! deps {
    () => {
        Wnaf!();
        WnafScalar!();
        Group!();
    };
}

macro_rules! WnafBase {
    () => {
        deps!();
        # [doc = " A fixed window table for a group element, precomputed to improve the speed of scalar"] # [doc = " multiplication."] # [doc = ""] # [doc = " This struct is designed for usage patterns that have long-term cached bases and/or"] # [doc = " scalars, or [Cartesian products] of bases and scalars. The [`Wnaf`] API enables one or"] # [doc = " the other to be cached, but requires either the base window tables or the scalar w-NAF"] # [doc = " forms to be computed repeatedly on the fly, which can become a significant performance"] # [doc = " issue for some use cases."] # [doc = ""] # [doc = " `WnafBase` and [`WnafScalar`] enable an alternative trade-off: by fixing the window"] # [doc = " size at compile time, the precomputations are guaranteed to only occur once per base"] # [doc = " and once per scalar. Users should select their window size based on how long the bases"] # [doc = " are expected to live; a larger window size will consume more memory and take longer to"] # [doc = " precompute, but result in faster scalar multiplications."] # [doc = ""] # [doc = " [Cartesian products]: https://en.wikipedia.org/wiki/Cartesian_product"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " use group::{WnafBase, WnafScalar};"] # [doc = ""] # [doc = " let wnaf_bases: Vec<_> = bases.into_iter().map(WnafBase::<_, 4>::new).collect();"] # [doc = " let wnaf_scalars: Vec<_> = scalars.iter().map(WnafScalar::new).collect();"] # [doc = " let results: Vec<_> = wnaf_bases"] # [doc = "     .iter()"] # [doc = "     .flat_map(|base| wnaf_scalars.iter().map(|scalar| base * scalar))"] # [doc = "     .collect();"] # [doc = " ```"] # [doc = ""] # [doc = " Note that this pattern requires specifying a fixed window size (unlike previous"] # [doc = " patterns that picked a suitable window size internally). This is necessary to ensure"] # [doc = " in the type system that the base and scalar `Wnaf`s were computed with the same window"] # [doc = " size, allowing the result to be computed infallibly."] # [derive (Clone , Debug)] pub struct WnafBase < G : Group , const WINDOW_SIZE : usize > { table : Vec < G > , }
    };
}

WnafBase!();