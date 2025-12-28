macro_rules! OwnULETy {
    () => {
        # [doc = " Represents a VarULE-compatible type that would typically"] # [doc = " be found behind a `Cow<'a, _>` in the last field, and is represented"] # [doc = " roughly the same in owned and borrowed versions"] # [derive (Copy , Clone , Debug)] enum OwnULETy < 'a > { # [doc = " [T] where T: AsULE<ULE = Self>"] Slice (& 'a Type) , # [doc = " str"] Str , }
    };
}

OwnULETy!();