macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! BodyIter {
    () => {
        deps!();
        # [doc = " An owning iterator of a section body. Created by [`Body::into_iter`], yielding"] # [doc = " un-normalized (`key`, `value`) pairs."] pub struct BodyIter < 'event > (std :: vec :: IntoIter < Event < 'event > >) ;
    };
}

BodyIter!();