macro_rules! Memoizable {
    () => {
        # [doc = " The trait that needs to be implemented for each intl formatter that needs to be"] # [doc = " memoized."] pub trait Memoizable { # [doc = " Type of the arguments that are used to construct the formatter."] type Args : 'static + Eq + Hash + Clone ; # [doc = " Type of any errors that can occur during the construction process."] type Error ; # [doc = " Construct a formatter. This maps the [`Self::Args`] type to the actual constructor"] # [doc = " for an intl formatter."] fn construct (lang : LanguageIdentifier , args : Self :: Args) -> Result < Self , Self :: Error > where Self : std :: marker :: Sized ; }
    };
}

Memoizable!();