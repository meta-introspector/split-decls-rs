use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl IntlLangMemoizer {
    /// Create a new [`IntlLangMemoizer`] that is unique to a specific
    /// [`LanguageIdentifier`]
    pub fn new(lang: LanguageIdentifier) -> Self {
        Self {
            lang,
            map: RefCell::new(type_map::TypeMap::new()),
        }
    }
    /// `with_try_get` means `with` an internationalization formatter, `try` and `get` a result.
    /// The (potentially expensive) constructor for the formatter (such as `PluralRules` or
    /// `DateTimeFormat`) will be memoized and only constructed once for a given
    /// `construct_args`. After that the format operation can be run multiple times
    /// inexpensively.
    ///
    /// The first generic argument `I` must be provided, but the `R` and `U` will be
    /// deduced by the typing of the `callback` argument that is provided.
    ///
    /// I - The memoizable intl object, for instance a `PluralRules` instance. This
    ///     must implement the Memoizable trait.
    ///
    /// R - The return result from the callback `U`.
    ///
    /// U - The callback function. Takes an instance of `I` as the first parameter and
    ///     returns the R value.
    pub fn with_try_get<I, R, U>(&self, construct_args: I::Args, callback: U) -> Result<R, I::Error>
    where
        Self: Sized,
        I: Memoizable + 'static,
        U: FnOnce(&I) -> R,
    {
        let mut map = self
            .map
            .try_borrow_mut()
            .expect("Cannot use memoizer reentrantly");
        let cache = map
            .entry::<HashMap<I::Args, I>>()
            .or_insert_with(HashMap::new);
        let e = match cache.entry(construct_args.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let val = I::construct(self.lang.clone(), construct_args)?;
                entry.insert(val)
            }
        };
        Ok(callback(e))
    }
}
