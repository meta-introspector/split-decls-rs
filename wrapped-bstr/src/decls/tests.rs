macro_rules! tests {
    () => {
        # [cfg (all (test , feature = "std"))] mod tests ;
    };
}

tests!()